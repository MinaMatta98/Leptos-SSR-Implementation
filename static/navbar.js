// const navlinks = document.querySelectorAll("nav a");

const searchButton = document.getElementById("search");
const dropdown = document.getElementsByClassName("dropdown-menu").item(0);

const handleSearch = () => {
	const searchBox = document
		.getElementsByClassName("form-control me-2")
		.item(0);
	const inputValue = searchBox.value.trim();

	if (inputValue.length > 0) {
		const xhr = new XMLHttpRequest();
		xhr.open("POST", "/headings", true);
		xhr.setRequestHeader("Accept", "text/text");
		xhr.send(inputValue);

		xhr.onreadystatechange = () => {
			if (
				xhr.readyState === 4 &&
				xhr.status === 200 &&
				xhr.responseText !== ""
			) {
				const options = xhr.responseText
					.split("\n")
					.filter((option) => option !== "");

				if (options.length > 0) {
					let i = 0;
					dropdown.innerHTML = "";
					dropdown.classList.add("bg-dark", "my-3");

					options.forEach((option, index) => {
						const { link, search_string } = JSON.parse(option);
						const optionElement = document.createElement("li");
						const linkElement = document.createElement("a");
						const dividerElement = document.createElement("div");

						linkElement.addEventListener("mouseover", (e) => {
							e.target.classList.remove("text-muted");
							e.target.classList.add("text-light");
						});

						linkElement.addEventListener("mouseout", (e) => {
							e.target.classList.remove("text-light");
							e.target.classList.add("text-muted");
						});

						linkElement.href = link;
						linkElement.classList.add("py-2", "text-muted");
						linkElement.innerHTML = search_string;
						optionElement.classList.add("p-2");
						optionElement.appendChild(linkElement);
						dropdown.appendChild(optionElement);

						i += 1;
						if (i < options.length) {
							dividerElement.classList.add("dropdown-divider");
							dropdown.appendChild(dividerElement);
						}
					});
					dropdown.style.display = "block";
				}
			} else {
				dropdown.innerHTML = "";
				dropdown.style.display = "none";
			}
		};
	} else {
		dropdown.innerHTML = "";
		dropdown.style.display = "none";
	}
};

searchButton.addEventListener("click", () => {
	["click", "keyup"].forEach((event) => {
		const searchBox = document
			.getElementsByClassName("form-control me-2")
			.item(0);
		searchBox.addEventListener(event, handleSearch);
	});
});
