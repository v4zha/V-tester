import "./Test.css";
import { useState, useEffect, Key} from "react";
import { invoke } from "@tauri-apps/api";
import { Tests } from "../../common/Types";
import { Link } from "react-router-dom";

function VTest(props:any) {
    const [tests, SetTests] = useState({ test: [{ t_name: "no test", input: [""], output: [""] }] } as Tests);
    const {vtest,setVtest}=props.value;
    useEffect(() => {
        invoke('tests')
            .then(t => SetTests(t as Tests))
            .catch(err => console.log(err));
    }, []);
    return (
        <>
            <h1>Tests</h1>
            <ul>
                {tests.test.map(t => {
                    return (<div>
                        <Link to="/validate">
                            <button onClick={() => setVtest(t)}>
                                <li key={t.t_name as Key}>{t.t_name}</li>
                            </button>
                        </Link>
                    </div>);
                })}
            </ul>
        </>
    )
}
export default VTest;