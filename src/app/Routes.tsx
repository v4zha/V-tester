import {Route,Routes,BrowserRouter} from "react-router-dom";
import { useState } from "react";
import App from "./App";
import VTest from "../features/tester/Test";
import Validator from "../features/validator/Validator";
import { Test} from "../common/Types";
function VRoutes(){
    const [vtest,setVtest]=useState({ t_name: "no test", input: [""], output: [""] } as Test);
    return (
        <BrowserRouter>
        <Routes>
        <Route path="/" element={<App/>} />        
        <Route path="test" element={<VTest value={{vtest,setVtest}}/>}/>
        <Route path="validate" element={<Validator value={{vtest,setVtest}}/>}/>        
        </Routes>
        </BrowserRouter>
    )
}
export default VRoutes;