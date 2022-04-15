import {Route,Routes,BrowserRouter} from "react-router-dom";
import { useState } from "react";
import App from "./App";
import VTest from "../features/tester/Test";
import Validator from "../features/validator/Validator";
import { Test} from "../common/Types";
function VRoutes(){
    const [valid_test,setValidtest]=useState({ t_name: "no test", input: [""], output: [""] } as Test);
    return (
        <BrowserRouter>
        <Routes>
        <Route path="/" element={<App/>} />        
        <Route path="test" element={<VTest value={{valid_test,setValidtest}}/>}/>
        <Route path="validate" element={<Validator value={{valid_test,setValidtest}}/>}/>        
        </Routes>
        </BrowserRouter>
    )
}
export default VRoutes;