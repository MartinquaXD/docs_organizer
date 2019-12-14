import {actionCreatorFactory} from 'typescript-fsa';
import {ThunkExtraArgs} from "../types";
import {ThunkAction} from 'redux-thunk';
import {State} from "../reducer";

const actionCreator = actionCreatorFactory();
export const setLoaderState = actionCreator<boolean>("SET_LOADER_STATE");
export const setScannedText = actionCreator<string>("SET_SCANNED_TEXT");


export const uploadImage = (): ThunkAction<Promise<any>, State, ThunkExtraArgs, any> => {
    return async (dispatch, getState, {history}) => {
        dispatch(setLoaderState(true))
        try {
            //TODO image upload
        } catch (er) {
            console.error("er", er);
        }
        dispatch(setLoaderState(false))
    }
};
