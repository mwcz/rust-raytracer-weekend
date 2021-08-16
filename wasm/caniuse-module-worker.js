export default function supportsModuleWorkers() {
    let supportsModuleWorker = false;
    const workerURL = URL.createObjectURL(new Blob([""]));
    const options = {
        get type() {
            supportsModuleWorker = true;
        },
    };
    new Worker(workerURL, options).terminate();
    URL.revokeObjectURL(workerURL);
    return supportsModuleWorker;
}
