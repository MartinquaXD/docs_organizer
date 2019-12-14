import * as React from "react"
import "./style.scss"
import {ChangeEvent} from "react";
import * as CBOR from "cbor-js";

export interface UploadProps {

}

export class Upload extends React.Component<UploadProps> {

    handleUpload = (evt: ChangeEvent<HTMLInputElement>) => {
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

            console.log("file", sendData)
            let res = await fetch("/uploadImage", {
                method: "POST",
                headers: {
                    'Content-Type': 'application/octet-stream'
                },
                body: CBOR.encode(sendData)
            });

            let txt = await res.text();
            console.log("resp", txt)
        }

        reader.readAsArrayBuffer(file);
    }

    render() {
        return <div className="upload">
            <input type="file" id={"upload"} onChange={this.handleUpload}
            />
            upload
        </div>
    }
}