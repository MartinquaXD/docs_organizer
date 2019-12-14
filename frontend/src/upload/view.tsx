import * as React from "react"
import "./style.scss"
import {ChangeEvent} from "react";
import * as CBOR from "cbor-js";
import * as actions from "./actions"

export interface UploadProps {
    isLoading: boolean,
    scannedText: string,

    setLoaderState: typeof actions.setLoaderState,
    setScannedText: typeof actions.setScannedText,

}

export class Upload extends React.Component<UploadProps> {

    handleUpload = (evt: ChangeEvent<HTMLInputElement>) => {
        const {setLoaderState, setScannedText} = this.props
        setLoaderState(true)
        const file = evt.target.files[0];

        const reader = new FileReader();

        const sendData = {
            name: file.name,
            date: file.lastModified,
            size: file.size,
            type: file.type
        } as any

        // Wenn der Dateiinhalt ausgelesen wurde...
        reader.onload = async function (blob) {
            sendData.fileData = new Uint8Array(blob.target.result as ArrayBuffer)

            let res = await fetch("/uploadImage", {
                method: "POST",
                headers: {
                    'Content-Type': 'application/octet-stream'
                },
                body: CBOR.encode(sendData)
            });

            let txt = await res.text();

            setScannedText(txt)
            setLoaderState(false)
        }

        reader.readAsArrayBuffer(file);
    }

    render() {
        const {scannedText, isLoading} = this.props

        if (isLoading) {
            return <div>loading</div>
        } else {
            return <div className="upload">
                <input type="file" id={"upload"} onChange={this.handleUpload}/>
                <div className="text">{scannedText}</div>
            </div>
        }

    }
}