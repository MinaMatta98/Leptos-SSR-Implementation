if (document.getElementById("drop-area")) {
	const dropArea = document.getElementById("drop-area");
	const fileElem = document.getElementById("file-input");

	dropArea.addEventListener("dragenter", preventDefaults, false);
	dropArea.addEventListener("dragover", preventDefaults, false);
	dropArea.addEventListener("dragleave", preventDefaults, false);
	dropArea.addEventListener("drop", preventDefaults, false);
	dropArea.addEventListener("drop", handleDrop, false);

	fileElem.addEventListener("change", handleFiles, false);

	function preventDefaults(e) {
		e.preventDefault();
		e.stopPropagation();
	}

	function handleDrop(e) {
		let dt = e.dataTransfer;
		let files = dt.files;

		handleFiles(files);
	}

	function handleFiles(files) {
		if (files && files.length > 0) {
			fileElem.files = files;
			fileElem.dispatchEvent(new Event("change"));
		}
		console.log(fileElem);
		console.log(files);
	}

	// clear button upon reload
	window.onload = function () {
		document.querySelector("#file-input").value = "";
		document.querySelector("#submit-button").disabled = true;
	};

	const urlParams = new URLSearchParams(window.location.search);
	const success = urlParams.has("success");
	const large = urlParams.has("outofbounds");
	const failure = urlParams.has("failure");
	const remove = urlParams.has("remove");
	if (success) {
		document.getElementById("result").style.whiteSpace = "no-wrap";
		document.getElementById("result").style.display = "block";
		document.getElementById("result").childNodes[1].style.display = "unset";
		document.getElementById("result").childNodes[3].style.display = "unset";
	} else if (large) {
		document.getElementById("result").style.whiteSpace = "no-wrap";
		document.getElementById("result").style.display = "block";
		document.getElementById("result").childNodes[5].style.display = "unset";
		document.getElementById("result").childNodes[7].style.display = "unset";
	} else if (failure) {
		document.getElementById("result").style.whiteSpace = "no-wrap";
		document.getElementById("result").style.display = "block";
		document.getElementById("result").childNodes[9].style.display = "unset";
		document.getElementById("result").childNodes[11].style.display = "unset";
	} else if (remove) {
		document.getElementById("result").style.whiteSpace = "no-wrap";
		document.getElementById("result").style.display = "block";
		document.getElementById("result").childNodes[13].style.display = "unset";
		document.getElementById("result").childNodes[15].style.display = "unset";
	} else {
		document.getElementById("result").style.display = "none";
		document.getElementById("result").childNodes[1].style.display = "none";
		document.getElementById("result").childNodes[3].style.display = "none";
	}
}

document.querySelector("#file-input").addEventListener("change", function () {
	var submitButton = document.querySelector("#submit-button");
	var uploadLabel = document.querySelector("#uploadLabel");
	if (this.files.length > 0) {
		submitButton.disabled = false;
		submitButton.classList.add("submit-button", "submit-button-large");
		uploadLabel.classList.add("uploadLabel-small");
		submitButton.style.display = "";
	} else {
		submitButton.disabled = true;
		submitButton.classList.remove("submit-button");
		this.classList.remove("uploadLabel-small");
		submitButton.style.display = "none";
	}
});

if (document.getElementById("remove-button")) {
	let button = document.getElementById("remove-button");
	// Add a click event listener to the button
	button.addEventListener("click", function () {
		// Create a FormData object to send with the request
		let src = document.getElementById("iframe").src;
		let file = src.split("/").pop();

		// Send a POST request to the server
		fetch(`/remove/${file}`, {
			method: "POST",
		}).then((response) => {
			if (response.status === 200) {
				// get the redirect url
				location.href = "/augment.html?remove";
			}
		});
	});
}

if (document.getElementById("view-button")) {
	let button = document.getElementById("view-button");
	// Add a click event listener to the button
	button.addEventListener("click", function () {
		// Create a FormData object to send with the request
		let src = document.getElementById("iframe").src;
		let file = src.split("/").pop();

		document.getElementById("loading-animation").style.display = "block";
		document.getElementById("form").style.display = "none";

		let fetchTimeout = setTimeout(() => {
			// If the fetch request takes longer than 5 seconds, retry
			fetch("/view", {
				headers: {
					"Content-Type": "application/x-www-form-urlencoded",
				},
				method: "POST",
				body: `index=${encodeURIComponent(file)}`,
			}).then((response) => {
				console.log(response);
				location.href = "http://localhost:8080/";
			});
		}, 2500);

		// Send a POST request to the server
		fetch("/view", {
			headers: {
				"Content-Type": "application/x-www-form-urlencoded",
			},
			method: "POST",
			body: `index=${encodeURIComponent(file)}`,
		}).then((response) => {
			console.log(response);
			location.href = "http://localhost:8080/";
			clearTimeout(fetchTimeout); // Clear the timeout if the request is successful
		});
	});
}

// selecting loading div
// const loader = document.querySelector("#loading");

// hiding loading
// function displayLoading() {
// 	loader.style.display = "inline-block";
// }

// hiding loading
// function hideLoading() {
// 	loader.style.display = "none";
// }

function populateThumbnails(data) {
	let arr = [];

	data.item.forEach((item) => {
		arr.push(item);
		let id = item.split("/").pop().split(".")[0];
		let thumbnailSection = document.getElementById("thumbnails");
		const img = document.createElement("img");
		img.src = `/${item}`;
		img.id = id;
		// img.style.width = "auto";
		// img.style.display = "block";
		img.classList.add("loading");
		thumbnailSection.appendChild(img);
	});

	while (arr.length > 0) {
		let id = arr[0].split("/").pop().split(".")[0];

		document.getElementById(`${id}`).addEventListener("load", (event) => {
			// element.style.animation = "none";
			event.target.classList.remove("loading", "p-2");
			// event.target.style = "height:140px; width:auto;";
			event.target.classList.add("mx-auto", "m-3");
		});

		document.getElementById(`${id}`).addEventListener("click", () => {
			document.getElementById("iframe").src = `/pdf/${id}`;
		});
		arr.splice(0, 1);
	}
}

window.addEventListener("DOMContentLoaded", () => {
	// displayLoading();
	fetch("/upload/list", {
		method: "GET",
	})
		.then((response) => {
			// hideLoading();
			let thumbnailSection = document.getElementById("thumbnails");
			thumbnailSection.classList.add("p-5", "mw-25");
			return response.json();
		})
		.then((data) => {
			populateThumbnails(data);
		});
});

let submitButton = document.querySelector("#submit-button");

submitButton.addEventListener("click", () => {
	document.getElementById("loading-animation").style.display = "block";
	document.getElementById("form").style.display = "none";
});

const queryString = window.location.search;
const urlParams = new URLSearchParams(queryString);
let arr = ["failure", "systemfailure", "success", "remove"];
arr.forEach((item) => {
	// console.log("here");
	// console.log(item);
	// courlParams.entries().next()nsole.log(urlParams.entries().next().value[0]);
	if (urlParams.entries().next().value !== undefined) {
		if (urlParams.entries().next().value[0] === item) {
			let status = document.getElementById("status");

			status.style.display = "block";

			let userStatus = document.createElement("p");
			let icon = document.createElement("i");

			if (item === "failure") {
				icon.classList.add("fa-solid", "fa-circle-xmark", "p-1", "text-danger");
				userStatus.textContent =
					"Failed to upload PDF. Encoding is likely not UTF-8 or PDF is corrupt";
			} else if (item === "success") {
				icon.classList.add(
					"fa-solid",
					"fa-circle-check",
					"p-1",
					"text-success",
				);
				userStatus.textContent = "PDF successfully uploaded";
			} else if (item === "systemfailure") {
				icon.classList.add("fa-solid", "fa-circle-xmark", "p-1", "text-danger");
				userStatus.textContent =
					"Internal Server Error. It is likely that the PDF cannot be decoded appropriately into a graph.";
			} else if (item === "remove") {
				icon.classList.add(
					"fa-solid",
					"fa-circle-check",
					"p-1",
					"text-success",
				);
				userStatus.textContent = "PDF successfully removed";
			}

			status.appendChild(userStatus);
			status.appendChild(icon);
		}
	}
});
