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
        vtest:Test,
        setVtest:React.Dispatch<React.SetStateAction<Test>>,
    }
}