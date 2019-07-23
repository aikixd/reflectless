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

pub trait FnContext<T>
{
    fn extract(&self) -> T;
}

pub trait Binder<'ctx, TCtx, TParam>
{
    fn make_params(&self, ctx: &'ctx TCtx) -> TParam;
}

pub trait TupleArgs<TParam, TResult>
{
    fn call(&self, params: TParam) -> TResult;
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

macro_rules! tuple_args_impl {
    ( $( ( $type:ident $name:ident) )* ) => {
        impl<Func, $( $type, )* TResult> TupleArgs<($( $type, )*), TResult> for Func
            where Func : Fn($( $type, )*) -> TResult
        {
            fn call(&self, ( $( $name, )* ) : ( $( $type, )* )) -> TResult
            {
                self($( $name, )*)
            }
        }
    };

    () => {};
}

macro_rules! tuple_args {
    ( ( $h_type:ident $h_name:ident ) $( ( $t_type:ident $t_name:ident ) )* ) => {
        tuple_args_impl!(( $h_type $h_name ) $( ( $t_type $t_name ) )* );
        tuple_args!( $( ( $t_type $t_name ) )* );
    };

    () => {};
}

tuple_args!((T1 t1) (T2 t2) (T3 t3) (T4 t4) (T5 t5) (T6 t6) (T7 t7) (T8 t8));
/*
impl<Func, T1, T2, TResult> TupleArgs<(T1, T2,), TResult> for Func
    where Func : Fn(T1, T2) -> TResult
{
    fn call(&self, params: (T1, T2)) -> TResult
    {
        self(params.0, params.1)
    }
}


*/