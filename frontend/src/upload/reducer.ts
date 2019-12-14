import * as actions from "./actions"
import {reducerWithInitialState} from "typescript-fsa-reducers"
import {LOCATION_CHANGE, RouterState} from "connected-react-router";

import actionCreatorFactory from "typescript-fsa";

const actionCreator = actionCreatorFactory();

export interface UploadState {
    scannedText: string,
    isLoading: boolean
}

const defaultState: UploadState = {
    scannedText: "",
    isLoading: false
};


const locationChange = actionCreator<RouterState>(LOCATION_CHANGE);

export const UploadReducer = reducerWithInitialState(defaultState)
    .case(actions.setScannedText, (state, payload) => {
        return {
            ...state,
            scannedText: payload
        }
    })
    .case(actions.setLoaderState, (state, payload) => {
        return {
            ...state,
            isLoading: payload
        }
    })