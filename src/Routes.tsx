import {Route,BrowserRouter} from "react-router-dom";
import App from "./App";
import Test from "./Test";
function VRoutes(){
    return (
        <BrowserRouter>
        <Route path="/" element={<App />} />
        <Route path="test" element={<Test/>}/>
        </BrowserRouter>
    )
}
export default VRoutes;