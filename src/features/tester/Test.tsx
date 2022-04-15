import "./Test.css";
import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api";
import { Vtest } from "../../common/Types";
import { Link } from "react-router-dom";
import { Button, Box } from "@mui/material";

function VTest(props: any) {
    const init_val: Vtest = { info: { name: "No info", desc: "No info" }, tests: { test: [{ t_name: "no test", input: [""], output: [""] }] } };
    const [vtests, SetTests] = useState(init_val);
    const { valid_test, setValidtest } = props.value;
    useEffect(() => {
        invoke('tests')
            .then(t => SetTests(t as Vtest))
            .catch(err => console.log(err));
    }, []);
    function test_info() {
        return (
            <div>
                Test Name : {vtests.info.name}<br />
                Test Description : {vtests.info.desc}<br />
            </div>
        )
    }
    return (
        <>
            <div>
                <h1 style={{ color: "#5e81ac" }}>V Tester</h1>
            </div>
            <Box sx={{ display: 'flex',flexDirection: 'column',gap:'20px' }}>
                    <h3>Tests</h3>
                    {test_info()}
                    <Box>
                        {vtests.tests.test.map(t => {
                            return (<Box className="test_div">
                                <Link to="/validate" style={{ textDecoration: 'none' }}>
                                    <Button className="t_button" onClick={() => setValidtest(t)} variant='contained' size="medium" sx={[{ background: '#d08770', color: '#2e3440' }, { '&:hover': { background: '#3b4252',color:'#d8dee9' } }]}>
                                        <span>{t.t_name}</span>
                                    </Button>
                                </Link>
                            </Box>);
                        })}
                    </Box>
                    <Link to="/" style={{ textDecoration: 'none' }}>
                        <Button className="test_button" variant='contained' size="large" sx={[{ background: '#5e81ac', color: '#d8dee9' }, { '&:hover': { background: '#3b4252' } }]}>
                            Home
                        </Button>
                    </Link>
            </Box>
        </>
    )
}
export default VTest;