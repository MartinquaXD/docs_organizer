import {UploadState, UploadReducer} from "./upload/reducer"

export interface State {
    upload: UploadState,

}

export const Reducer =  {
    upload: UploadReducer,
}