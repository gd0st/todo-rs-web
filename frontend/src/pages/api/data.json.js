import { AstroComponentInstance } from "astro/runtime/server/index.js";

let RUST_URL = "http://127.0.0.1:8080/add_todo"
export async function POST({request}) {
	console.log("received!");
	console.log("new new")
	console.log(await request.body)
	let todo = {
		subject: "Dishes!!",
		id: null,
		status: "Stopped",
	}
	console.log(todo)

	try {
		let res = await fetch(RUST_URL, {
			method: "POST",
			body: JSON.stringify(todo)
		})
	} catch(error) {

		if (error instanceof Error) {
			console.error(error.message)
		}

	}
	return new Response(JSON.stringify({
		message: "New Todo Sent."
	}), {
		status: 200,
		headers: {
			"Content-Type": "application/json",
		}
	})


}