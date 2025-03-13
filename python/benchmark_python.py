import asyncio
import sys

# Benchmark Python
async def main(num_tasks: int):
    tasks = []
    for task_id in range(num_tasks):
        tasks.append(asyncio.sleep(10))
    await asyncio.gather(*tasks)
    print("shutdown.")

if __name__ == "__main__":
    num_task = int(sys.argv[1])
    print(f"startup: {num_task} task")
    asyncio.run(main(num_task))