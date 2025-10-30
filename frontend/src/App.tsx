import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import { ThemeProvider } from '@mui/material/styles';
import CssBaseline from '@mui/material/CssBaseline';
import Layout from './components/Layout';
import Home from './components/Home';
import SqlEditor from './components/SqlEditor';
import ServiceView from './components/ServiceView';
import Kafka from './components/Kafka';
import theme from './theme';

// theme comes from ./theme

function App() {
  return (
    <ThemeProvider theme={theme}>
      <CssBaseline />
      <Router basename="/dev/ui">
        <Routes>
          <Route path="/" element={<Layout />}>
            <Route index element={<Home />} />
            <Route path="sql" element={<SqlEditor />} />
            <Route path="kafka" element={<Kafka />} />
            <Route path="service/:serviceName" element={<ServiceView />} />
          </Route>
        </Routes>
      </Router>
    </ThemeProvider>
  );
}

export default App;