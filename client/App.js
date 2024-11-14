import React from "react";
import { BrowserRouter as Router, Route, Switch } from "react-router-dom";
import UploadPage from "./UploadPage";
import FilesOverviewPage from "./FilesOverviewPage";
import ShareFilePage from "./ShareFilePage";

class SecureFileSharingApp extends React.Component {
  render() {
    return (
      <Router>
        <div>
          <Switch>
            <Route path="/upload" component={UploadPage} />
            <Route path="/files" component={FilesOverviewPage} />
            <Route path="/share" component={ShareFilePage} />
            <Route path="/" exact component={FilesOverviewPage} />
          </Switch>
        </div>
      </Router>
    );
  }
}

export default SecureFileSharingApp;