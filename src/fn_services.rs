use std::marker::PhantomData;

pub struct FnBinding<'ctx, TFn, TCtx, TParam, TResult>
    where 
        TFn : Fn(TParam) -> TResult
{
    func: TFn,
    ctx: &'ctx TCtx,
    phantom_param: PhantomData<TParam>,
    phantom_result: PhantomData<TResult>
}

impl<'ctx, TFn, TCtx, TParam, TResult> FnBinding<'ctx, TFn, TCtx, TParam, TResult>
    where 
        TFn : Fn(TParam) -> TResult + Binder<'ctx, TCtx, TParam>
{
    pub fn bind(ctx : &'ctx TCtx, func: TFn) -> FnBinding<'ctx, TFn, TCtx, TParam, TResult>
        where
    {
        FnBinding
        {
            func,
            ctx,
            phantom_param: PhantomData,
            phantom_result: PhantomData
        }
    }

    pub fn call(&self) -> TResult
    {
        let func = &self.func;
        
        let prm = func.make_params(&self.ctx);

        func(prm)
    }
}


pub trait Binder<'ctx, TCtx, TParam>
{
    fn make_params(&self, ctx: &'ctx TCtx) -> TParam;
}

impl<'ctx, TCtx, Func, TResult, T1> Binder<'ctx, TCtx, (T1,)> for Func
    where
        Func : Fn((T1,)) -> TResult,
        TCtx : FnContext<T1>
{
    fn make_params(&self, ctx: &'ctx TCtx) -> (T1,)
    {
        (
            (ctx as &FnContext<T1>).extract(),
        )
    }
}

impl<'ctx, TCtx, Func, TResult, T1, T2> Binder<'ctx, TCtx, (T1, T2)> for Func
    where
        Func : Fn((T1, T2)) -> TResult,
        TCtx : FnContext<T1> + FnContext<T2>
{
    fn make_params(&self, ctx: &'ctx TCtx) -> (T1, T2)
    {
        (
            (ctx as &FnContext<T1>).extract(),
            (ctx as &FnContext<T2>).extract(),
        )
    }
}

pub trait FnContext<T>
{
    fn extract(&self) -> T;
}



#[cfg(test)]
mod tests 
{
    use crate::fn_services::*;

    struct Ctx
    {

    }

    impl FnContext<i32> for Ctx
    {
        fn extract(&self) -> i32
        {
            2
        }
    }

    impl FnContext<String> for Ctx
    {
        fn extract(&self) -> String
        {
            String::from("hello")
        }
    }

    fn fn_1(param: (i32,)) -> i32
    {
        param.0
    }

    fn fn_2(param: (i32, String)) -> String
    {
        let num = param.0.to_string();

        num + &param.1
    }
    
    #[test]
    fn apply_1() 
    {
        let ctx = Ctx 
        {

        };
        
        let applied = FnBinding::bind(&ctx, fn_1);
        
        assert_eq!(applied.call(), 2);
    }

    #[test]
    fn apply_2()
    {
        let ctx = Ctx 
        {

        };
        
        let applied = FnBinding::bind(&ctx, fn_2);
        
        assert_eq!(applied.call(), "2hello");
    }
}