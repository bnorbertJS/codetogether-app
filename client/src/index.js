import React from "react";
import ReactDOM from "react-dom";

const App = ({name}) => <h1> {name} </h1>

ReactDOM.render(
    <App name="CodeTogether!" />,
    document.getElementById("app")
)