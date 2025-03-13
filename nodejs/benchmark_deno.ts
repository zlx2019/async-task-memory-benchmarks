async function runTasks(numTasks: number) {
    console.log(`startup: ${numTasks} task.`);
    const tasks = [];
    for (let i = 0; i < numTasks; i++) {
      tasks.push(new Promise(resolve => setTimeout(resolve, 10000)));
    }
    await Promise.all(tasks);
    console.log("shutdown.");
  }
  
  // Deno 命令行参数获取
  const numTasks = parseInt(Deno.args[0]);
  runTasks(numTasks);
  