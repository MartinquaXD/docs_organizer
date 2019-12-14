console.log("run app")

import * as React from "react";

import {Provider, connect} from 'react-redux'

import * as ReactDom from "react-dom";

import "./scss/app.scss";
import {createStore, applyMiddleware, combineReducers, bindActionCreators, compose} from 'redux';
import thunk from 'redux-thunk';
import {Reducer, State} from "./reducer";
import {Router, Route, Redirect, RouteProps} from 'react-router'
import {Switch} from 'react-router-dom';


import {createBrowserHistory} from "history"

const history = createBrowserHistory();
import {ConnectedRouter, routerMiddleware, connectRouter} from 'connected-react-router';

const reducers = combineReducers({...Reducer, router: connectRouter(history)});
const composeEnhancers = (window as any).__REDUX_DEVTOOLS_EXTENSION_COMPOSE__ || compose;

const store = createStore(reducers, composeEnhancers(
    applyMiddleware(thunk.withExtraArgument({history}), routerMiddleware(history))
    )
)

import {UploadContainer} from "./upload/container"


class AppWrapper extends React.Component {
    render() {
        return <Switch>
            <Route path="/upload" render={props => <UploadContainer {...props} />}/>
            {<Redirect from="*" to="/"/>}
        </Switch>
    }
}

class App extends React.Component {
    render() {
        return <Provider
            store={store}>
            <ConnectedRouter history={history}>
                <Route path="/" render={props => <AppWrapper/>}/>
            </ConnectedRouter>
        </Provider>
    }
}

ReactDom.render(<App/>, document.getElementById("app"));