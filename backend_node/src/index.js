import express  from "express"
import { loginRoute, registerRoute } from "./routes/auth"

const app = express()

app.get("/", (req, res) =>  res.send("Well... this is one hell of LANDING PAGE!!!!"))
app.get("/login", loginRoute )
app.get("/register", registerRoute )

app.listen(3000, () => console.log("running on port 3000 "))

