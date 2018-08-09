import assert from "assert"
import axios from "axios"
import "../src/index.js"

describe("CodeTogether init", () => {
    it("should return http 200", done => {
        axios.get("http://localhost:3000")
            .then( resp => { 
                assert.equal(200, resp.status)
                done()
            })
    })
})
