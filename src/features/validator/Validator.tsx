import { TestProp } from "../../common/Types";
import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api";
import Button from '@mui/material/Button';
import { Link } from "react-router-dom";
import "./Validator.css";

function Validator(props: TestProp) {
    const [test_res, setTestRes] = useState(false);
    const { valid_test, setValidtest } = props.value;
    useEffect(() => {
        invoke('validate', { tName: valid_test.t_name })
            .then(res => setTestRes(res as boolean))
            .catch(err => console.log(err));
    }, []);
    return (
        <>
            <div>
                <h1 style={{ color: "#5e81ac" }}>V Tester</h1>
            </div>
            <div className="test_name">
                <h3>Validating Test [ {valid_test.t_name} ] </h3>
            </div>
            <div className="t_result">
                <h4>Test Result : [ {JSON.stringify(test_res)} ] </h4>
            </div>
            <div>
                <Link to="/test" style={{ textDecoration: 'none' }}>
                    <Button className="test_button" variant='contained' sx={[{ background: '#5e81ac', color: '#d8dee9' }, { '&:hover': { background: '#3b4252' } }]} >
                        Go back
                    </Button>
                </Link>
            </div>
        </>
    )
};
export default Validator;
