<script>
	import Button from "./components/button.svelte";
</script>

<main style = "width: 800px; height: 600px; position: fixed; top: 0; left: 0" class = "bg-gray-100" >
	<div class = "">
		// Logo resides here.	
		<small> Login to CrabbyDNS</small>	
		<br />
		<form method = "POST">
			<input class = "border-none border-red-200 filter outline-none drop-shadow-sm focus:outline-none focus:ring-4 focus:ring-red-200 focus:ring-opacity-50 rounded-md p-2 m-2 w-64 font-normal pl-4 text-sm" placeholder="Username"/><br />
			<input class = "border-none border-red-200 filter outline-none drop-shadow-sm focus:outline-none focus:ring-4 focus:ring-red-200 focus:ring-opacity-50 rounded-md p-2 m-2 w-64 font-normal pl-4 text-sm" placeholder="Password" type="password"/><br />

			<br />
			
			<Button label = "Login"></Button>

		</form>
	</div>
</main>