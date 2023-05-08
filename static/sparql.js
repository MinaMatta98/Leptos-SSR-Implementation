// Get form data
const submitButton = document.getElementById("sparql");
const form = document.getElementById("sparql-form");

form.addEventListener("submit", function(event) {
	event.preventDefault();
	event.stopPropagation();
});

submitButton.addEventListener("click", function(event) {
	event.preventDefault();
	event.stopPropagation();

	const subject = document.getElementById("sparql").value;

	if (subject) {
	// Create HTTP request
	const xhr = new XMLHttpRequest();
	xhr.open("POST", "/sparql", true);
	xhr.setRequestHeader("Accept", "application/sparql-results+json");
	xhr.setRequestHeader("Content-Type", "application/sparql-query");
	xhr.send(subject);
	xhr.onreadystatechange = function() {
		if (this.readyState === 4 && this.status === 200) {
			// Parse the response as JSON
			let data = JSON.stringify(xhr.responseText).split();
			// Manipulate the HTML DOM to place the data within the body
			let body = document.getElementById("sparql-showcase-text");
			let parent = document.getElementById("sparql-showcase");
			parent.style.display = "block";
			// body.style.display = "block";
			body.innerText = `${data}`;
		}
	};
	}
});
