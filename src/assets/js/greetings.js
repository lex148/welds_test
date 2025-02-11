
import { Controller } from "https://unpkg.com/@hotwired/stimulus/dist/stimulus.js";

class GreetingController extends Controller {

	greet() {
		alert("Hello !!!");
	}

}

Stimulus.register("greeting", GreetingController);
