import { AstroComponentInstance } from "astro/runtime/server/index.js";

function endpoint(s) {
	let base_url = "http://127.0.0.1:8080"
	let endpoints = new Map();
	endpoints.set("addTodo", "/add_todo");
	endpoints.set("todos", "/todos");

	if (typeof s == "string" && endpoints.has(s)) {
		let endpoint = endpoints.get(s);
		return `${base_url}${endpoint}`;
	}
	return null;
}

export async function POST({request}) {

	let url = endpoint("addTodo");

	if ( url == null) {
		return new Response(JSON.stringify({message: "failed to find endpoint."}), {
			status: 501,
			headers: {
				"Content-type": "application/json"
			}
		})
	}

	let todo = await request.json()

	try {
		let res = await fetch(url, {
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