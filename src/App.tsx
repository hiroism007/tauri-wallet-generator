import React, { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { open } from '@tauri-apps/api/dialog';
import {  desktopDir } from '@tauri-apps/api/path';
import { message } from '@tauri-apps/api/dialog';
import "./App.css";

function App() {
    const [number, setNumber] = useState("");
    const [dirPath, setDirPath] = useState("");
    const [loading, setLoading] = useState(false);

    async function generate() {
        setLoading(true)
        try {
            const res : string = await invoke("generate", {numberOfWallet: Number(number), dirPath })
            await message(res);
        } catch (e) {
            console.error(e);
            await message('Hmm..', {type: 'error'})
        } finally {
            setLoading(false)
        }

    }

    function openDialog () {
        open({ directory: true, multiple: false }).then(files => {
            setDirPath(files as string)
        })
    }

    React.useEffect(() => {
        desktopDir().then(r => setDirPath(r))
    }, [])

    return (
        <div className="container">

            <h1>Welcome to Bulk Ethereum Wallet Generator!</h1>

            <div className="loader" hidden={!loading}>
                <h1>GENERATING..</h1>
                <span></span>
                <span></span>
                <span></span>
            </div>

            <div hidden={loading}>
                <p>
                    Select the path to download wallet files.
                </p>

                <div className="row">
                    <input
                        id="greet-input"
                        value={dirPath}
                        readOnly={true}
                    />
                    <button type="button" onClick={() => openDialog()}>
                        Select
                    </button>
                </div>

                <p>
                    Enter the number of wallets you want to generate.
                </p>

                <div className="row">
                    <div>
                        <input
                            id="greet-input"
                            onChange={(e) => setNumber(e.currentTarget.value)}
                            placeholder="Enter the number of..."
                        />
                        <button type="button" onClick={() => generate()}>
                            Generate
                        </button>
                    </div>
                </div>
            </div>


        </div>
    );
}

export default App;
