import {connect} from 'react-redux'
import {bindActionCreators} from 'redux'
import * as actions from "./actions"
import {mergePropsFunc} from "../types"
import {State} from "../reducer"

import {UploadProps, Upload} from "./view"

import {RouteComponentProps, withRouter} from "react-router"

export interface UploadContainerProps extends RouteComponentProps<{ subjectId: string }> {
}


const mapStateToProps = (state: State, ownProps: UploadContainerProps) => {
    const {upload} = state

    return {
        isLoading: upload.isLoading,
        scannedText: upload.scannedText
    }
}


const mapDispatchToProps = (dispatch: any, ownProps: UploadContainerProps) => {
    return bindActionCreators({
        ...actions
    }, dispatch)
}


export const UploadContainer = withRouter(connect<ReturnType<typeof mapStateToProps>, ReturnType<typeof mapDispatchToProps>, UploadContainerProps, UploadProps, State>(
    mapStateToProps,
    mapDispatchToProps,
    mergePropsFunc
)(Upload))