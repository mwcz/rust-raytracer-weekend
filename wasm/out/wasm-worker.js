var a,s=!1;console.log("wasm-worker module");addEventListener("message",async r=>{if(r.data==="init")try{let e=await import("./wasm-render.js");a=e.wasmRender,await e.wasmInit(),s=!0,postMessage({status:"success",data:{initialized:s}})}catch(e){console.error(e),postMessage({status:"error",message:"error occurred during web worker import"})}else if(r.data==="render")try{let e=await a();postMessage({status:"success",data:{initialized:s,imageData:e}})}catch(e){console.error(e),postMessage({status:"error",message:"error occurred during render"})}});