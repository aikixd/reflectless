//#[cfg(test)]
mod tests 
{
    use reflectless::fn_services::*;

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

    fn fn_1(param: i32) -> i32
    {
        param
    }

    fn fn_2(i: i32, s: String) -> String
    {
        let num = i.to_string();

        num + &s
    }
    
    #[test]
    fn apply_1() 
    {
        let ctx = Ctx 
        {

        };
        
        let applied = FnBinding::new(&ctx, fn_1);
        
        assert_eq!(applied.call(), 2);
    }

    #[test]
    fn apply_2()
    {
        let ctx = Ctx 
        {

        };
        
        let applied = FnBinding::new(&ctx, fn_2);
        
        assert_eq!(applied.call(), "2hello");
    }
}