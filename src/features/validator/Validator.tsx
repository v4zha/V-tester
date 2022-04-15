import { TestProp } from "../../common/Types";
import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api";
import "./Validator.css";
function Validator(props:TestProp) {
    const [test_res,setTestRes]=useState(false);
    const {vtest,setVtest}=props.value;
    useEffect(() => {
        invoke('validate',{tName:vtest.t_name})
            .then(res=> setTestRes(res as boolean))
            .catch(err => console.log(err));
    }, []);
    return (
        <>
            <h3>Validating Test [ {vtest.t_name} ] </h3>
            <h4>Test Result : [ {JSON.stringify(test_res)} ] </h4>
        </>
    )
};
export default Validator;
