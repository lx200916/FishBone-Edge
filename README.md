<p align="center">

<img src="https://res.craft.do/user/full/5e6b3682-4335-44ac-eb7c-2a35a30ff739/doc/116DC87E-A004-4CDE-9CDB-DF508E71C918/D2F42156-D7D7-4867-9A0B-65DEF3F54FB5_2/dMyya0f0OBrkroLLbQBp35ezpo0yqxHjWFdtpgiUj3gz/Untitled%201.png" style="width:120px;border-radius:20px;"/>

</p>

<div align="center">

# FishBone Edge Gateway

🚀 Fetch & Decrypt Paste on the Edge, powered by Cloudflare Workers & Rust.

We deploy the service to offer a **read-only** API endpoint for Headless Users or cURL Lovers.


</div>






## Uasge

You can access the CF Worker service deployed at `https://api.bone.saltedfish.fun`.

> **Warning**
>
> The API endpoint is read-only, which means you can only `Get Paste`, `Create/Remove Paste` action is forbidden.

For Normal Paste:

```other
https://api.bone.saltedfish.fun/_api/paste/:id
```

For Encrypted Paste:

```other
https://api.bone.saltedfish.fun/_api/paste/:id?p={password}
```

> **Note**
>
> The Feature `Burn After Reading`  on **encrypted paste** is disabled in the service, since we notice API can be incidentally fetched by some prefetch services when the link transfered among IMs.

