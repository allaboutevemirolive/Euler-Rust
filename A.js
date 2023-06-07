const { chromium } = require('playwright');

async function scrapeData() {
  const browser = await chromium.launch( { headless: false });
  const context = await browser.newContext();
  const page = await context.newPage();

  // Navigate to the target link
  await page.goto('https://projecteuler.net/archives');

  // Get all the tr elements
  const trElements = await page.$$('tr');

  // Iterate over each tr element and extract the problem number and title
  for (const trElement of trElements) {
    const [numberElement, titleElement] = await trElement.$$('td');

    // Extract the problem number and title
    const problemNumber = await numberElement.textContent();
    const problemTitle = await titleElement.textContent();

    // Convert the scraped elements to strings
    const problemNumberString = problemNumber.toString();
    const problemTitleString = problemTitle.toString();

    // Log the scraped data
    console.log('Problem Number:', problemNumberString);
    console.log('Problem Title:', problemTitleString);
  }

  await browser.close();
}

scrapeData();
