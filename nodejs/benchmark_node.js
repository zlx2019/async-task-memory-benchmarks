const util = require("util");
const delay = util.promisify(setTimeout);

async function runTasks(numTasks) {
  console.log(`startup: ${numTasks} task.`);
  const tasks = [];
  for (let i = 0; i < numTasks; i++) {
    tasks.push(delay(10000));
  }
  await Promise.all(tasks);
  console.log("shutdown.");
}

const numTasks = parseInt(process.argv[2]);
runTasks(numTasks);
