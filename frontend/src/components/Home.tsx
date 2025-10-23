import { useNavigate } from 'react-router-dom';

const Home = () => {
  const navigate = useNavigate();

  const handleSqlEditorClick = () => {
    navigate('/sql');
  };

  return (
    <div className="devui-container">
      <div className="container">
        <h1>DevUI - Development Tools</h1>
        <div className="welcome">
          Welcome to your development tools dashboard. Select a tool below to get started.
        </div>

        <div className="tool-navigation">
          <h2>Available Tools</h2>

          <div className="tool-card" onClick={handleSqlEditorClick}>
            <div className="tool-header">
              <div className="tool-icon">SQL</div>
              <div className="tool-info">
                <h3>SQL Editor</h3>
                <p>Query and manage your PostgreSQL databases</p>
              </div>
            </div>
            <div className="tool-description">
              Execute SQL queries, browse database schemas, and manage your PostgreSQL databases with a powerful SQL editor interface.
            </div>
          </div>

          <div className="tool-card disabled">
            <div className="tool-header">
              <div className="tool-icon">K</div>
              <div className="tool-info">
                <h3>Kafka UI</h3>
                <p>Monitor and manage Kafka topics</p>
              </div>
            </div>
            <div className="tool-description">
              Coming soon: Monitor and manage your Kafka topics and messages.
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default Home;
