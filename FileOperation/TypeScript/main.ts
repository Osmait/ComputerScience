import { readFile, writeFile } from "fs/promises"
type Endpoint = {
  url: string;
}
async function main() {
  const file = await readFile("endpoint.json", "utf-8");
  const endpoint: Endpoint = JSON.parse(file)
  const response = await fetch(endpoint.url)
  const result = await response.json()
  console.log(result)
  await writeFile("response.json", JSON.stringify(result), 'utf-8')


}

main();
