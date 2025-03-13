import java.time.Duration;
import java.util.ArrayList;
import java.util.List;

/// 使用Java21 提供的虚拟线程进行基准测试
public class BenchmarkJava {
    public static void main(String[] args) throws InterruptedException {
        int numTasks = Integer.parseInt(args[0]);
        System.out.println("startup :" + numTasks + " tasks");
        List<Thread> tasks = new ArrayList<>(numTasks);
        for (int i = 0; i < numTasks; i++) {
            Thread task = Thread.startVirtualThread(()-> {
                try {
                    Thread.sleep(Duration.ofSeconds(10));
                } catch (InterruptedException e) {
                    throw new RuntimeException(e);
                }
            });
            tasks.add(task);
        }
        for (Thread task : tasks) {
            task.join();
        }
        System.out.println("shutdown.");
    }
}