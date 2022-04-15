import './App.css';
import { Link } from "react-router-dom";
import Button from '@mui/material/Button';
import Vlogo from '../features/Vlogo';
function App() {
  return (
    <div className="App">
      <header className="App-header">
        <Vlogo/>
        <Link to="/test" style={{ textDecoration: 'none' }}>
          <Button className="test_button" variant='contained' size="large" sx={[{ background: '#5e81ac', color: '#d8dee9' }, { '&:hover': { background: '#3b4252' } }]}>
            Get Tests
          </Button>
        </Link>
      </header>
    </div>
  )
}

export default App;
