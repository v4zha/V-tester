export interface Test{
    t_name:String,
    input:Array<String>,
    output:Array<String>,
}
export interface Tests{
    test:Array<Test>,
}
export type TestProp={
    value:{
        valid_test:Test,
        setValidtest:React.Dispatch<React.SetStateAction<Test>>,
    }
}
export interface Info{
    name:String,
    desc:String,
}
export interface Vtest{
    info:Info,
    tests:Tests,
}