const axios = require('axios')

async function HttpReverseProxySite({ port, siteID }) {

    const BASE_URL = process.env.PROXY_URL;

    try{
        const response = await axios.post(`${BASE_URL}/register`, {
            port: port,
            siteId: siteID
        })

        if(response.status === 201) {
            console.log(`Reverse proxy initialized for site ${siteID} on port ${port}`);
        }
    }catch(err){
        console.log(`Error during reverse proxy initialization for site ${siteID}:`, err);
    }
}

module.exports = { HttpReverseProxySite }