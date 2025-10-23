import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import Home from './components/Home';
import SqlEditor from './components/SqlEditor';
import './App.css';

function App() {
  return (
    <Router basename="/dev/ui">
      <Routes>
        <Route path="/" element={<Home />} />
        <Route path="/sql" element={<SqlEditor />} />
      </Routes>
    </Router>
  );
}

export default App;