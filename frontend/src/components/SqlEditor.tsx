import { useState, useEffect } from 'react';

interface Table {
  name: string;
  schema: string;
  columns: Array<{ name: string; type: string }>;
}

const SqlEditor = () => {
  const [tables, setTables] = useState<Table[]>([]);
  const [loading, setLoading] = useState(true);
  const [query, setQuery] = useState('');
  const [results, setResults] = useState<any[]>([]);
  const [executing, setExecuting] = useState(false);

  useEffect(() => {
    loadTables();
  }, []);

  const loadTables = async () => {
    try {
      const response = await fetch('/dev/ui/api/tables');
      const tablesData = await response.json();
      setTables(tablesData);
    } catch (error) {
      console.error('Failed to load tables:', error);
    } finally {
      setLoading(false);
    }
  };

  const executeQuery = async () => {
    if (!query.trim()) return;

    setExecuting(true);
    try {
      // TODO: Implement query execution API
      console.log('Executing query:', query);
      // For now, just show a placeholder
      setResults([{ message: 'Query execution not yet implemented' }]);
    } catch (error) {
      console.error('Failed to execute query:', error);
      setResults([{ error: 'Failed to execute query' }]);
    } finally {
      setExecuting(false);
    }
  };

  const clearQuery = () => {
    setQuery('');
    setResults([]);
  };

  return (
    <div className="sql-editor">
      <div className="sql-editor-header">
        <h2>SQL Editor - PostgreSQL</h2>
        <div className="connection-status">
          <div className="status-indicator"></div>
          <span>Connected</span>
        </div>
      </div>

      <div className="sql-editor-content">
        <div className="sql-sidebar">
          <h3 className="table-count">
            Database Tables ({tables.length})
          </h3>
          <div className="table-list">
            {loading ? (
              <div className="loading-tables">Loading tables...</div>
            ) : tables.length === 0 ? (
              <div className="no-tables">No tables found or database not connected</div>
            ) : (
              tables.map((table, index) => (
                <div key={index} className="table-item">
                  <div className="table-name">{table.name}</div>
                  <div className="table-schema">Schema: {table.schema}</div>
                  <div className="table-columns">Columns: {table.columns.length}</div>
                </div>
              ))
            )}
          </div>
        </div>

        <div className="sql-main">
          <div className="query-editor">
            <div className="editor-toolbar">
              <button
                className="btn btn-primary"
                onClick={executeQuery}
                disabled={executing || !query.trim()}
              >
                {executing ? 'Executing...' : 'Execute Query'}
              </button>
              <button className="btn btn-secondary" onClick={clearQuery}>
                Clear
              </button>
            </div>
            <textarea
              className="sql-textarea"
              placeholder="Enter your SQL query here...\n\nExample:\nSELECT * FROM users LIMIT 10;"
              value={query}
              onChange={(e) => setQuery(e.target.value)}
            />
          </div>

          <div className="query-results">
            <div className="results-header">Query Results</div>
            <div className="results-content">
              {results.length === 0 ? (
                <div className="no-results">
                  No query executed yet. Enter a SQL query above and click 'Execute Query'.
                </div>
              ) : (
                <pre>{JSON.stringify(results, null, 2)}</pre>
              )}
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default SqlEditor;
