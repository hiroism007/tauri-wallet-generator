import { useState, useEffect, useCallback, ChangeEventHandler, ChangeEvent } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { open } from '@tauri-apps/api/dialog';
import { desktopDir } from '@tauri-apps/api/path';
import { message } from '@tauri-apps/api/dialog';
import "./App.css";

function App() {
    const [number, setNumber] = useState("10");
    const [dirPath, setDirPath] = useState("");
    const [loading, setLoading] = useState(false);
    const [qrCode, setQRCode] = useState(false);
    const [csv, setCsv] = useState(false);

    async function generate() {
        setLoading(true)
        try {
            const res: string = await invoke("generate", {
                numberOfWallet: Number(number),
                dirPath,
                qrCode,
                csv
            })
            await message(res);
        } catch (e) {
            console.error(e);
            await message('Hmm..', {type: 'error'})
        } finally {
            setLoading(false)
        }

    }

    function openDialog() {
        open({directory: true, multiple: false}).then(files => {
            setDirPath(files as string)
        })
    }

    const updateCheckbox = useCallback((type: string) => {
        return (e: ChangeEvent<HTMLInputElement>) => {
            console.log(e.target.checked)
            if (type === 'csv') setCsv(e.target.checked)
            if (type === 'qr') setQRCode(e.target.checked)
        }
    }, [])

    useEffect(() => {
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
                    Check if you need a csv file or qrcode images. <br/>
                    Only JSON files will be exported by default.
                </p>

                <div className="row-vertical">
                    <div>
                        <input type="checkbox" id="csv" name="csv" value={Number(csv)}
                               onChange={updateCheckbox('csv')}/>
                        <label htmlFor="scales">export csv file</label>
                    </div>

                    <div>
                        <input type="checkbox" id="qrcode" name="qrcode" value={Number(qrCode)}
                               onChange={updateCheckbox('qr')}/>
                        <label htmlFor="horns">export qrcodes *takes long time</label>
                    </div>
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
                            value={number}
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
