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
    pub fn new(ctx : &'ctx TCtx, func: TFn) -> FnBinding<'ctx, TFn, TCtx, TParam, TResult>
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

pub trait FnContext<T>
{
    fn extract(&self) -> T;
}

pub trait Binder<'ctx, TCtx, TParam>
{
    fn make_params(&self, ctx: &'ctx TCtx) -> TParam;
}

macro_rules! binder_impl {
    ( $head:ident $( $tail:ident )* ) => {
        impl<'ctx, TCtx, Func, TResult, $head, $( $tail ),* > Binder<'ctx, TCtx, ( $head, $( $tail),* )> for Func
            where Func : Fn(( $head, $( $tail ),* )) -> TResult,
            TCtx : FnContext<$head> + $( FnContext<$tail> +)*
        {
            fn make_params(&self, ctx: &'ctx TCtx) -> ($head, $( $tail ),* )
            {
                (
                    (ctx as &FnContext<$head>).extract(),
                    $((ctx as &FnContext<$tail>).extract(),)*
                )
            }
        }

        binder_impl!( $( $tail )* );
    };

    () => {}
}

binder_impl!(T1 T2 T3 T4 T5 T6 T7 T8);

