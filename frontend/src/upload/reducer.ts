import * as actions from "./actions"
import {reducerWithInitialState} from "typescript-fsa-reducers"
import {LOCATION_CHANGE, RouterState} from "connected-react-router";

import actionCreatorFactory from "typescript-fsa";

const actionCreator = actionCreatorFactory();

export interface UploadState {
}

const defaultState: UploadState = {
};


const locationChange = actionCreator<RouterState>(LOCATION_CHANGE);

export const UploadReducer = reducerWithInitialState(defaultState)