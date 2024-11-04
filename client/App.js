import React from "react";
import { BrowserRouter as Router, Route, Switch } from "react-router-dom";
import FileUpload from "./FileUpload";
import FileList from "./FileList";
import FileShare from "./FileShare";
class App extends React.Component {
  render() {
    return (
      <Router>
        <div>
          <Switch>
            <Route path="/upload" component={FileUpload} />
            <Route path="/files" component={FileList} />
            <Route path="/share" component={FileShare} />
            <Route path="/" exact component={FileList} />
          </Switch>
        </div>
      </Router>
    );
  }
}
export default App;