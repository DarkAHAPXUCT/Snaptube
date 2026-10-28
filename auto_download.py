import subprocess
import time
import json
import sys

print("⏳ Ждем завершения сборки на GitHub (это займет несколько минут)...")
print("Скрипт сам скачает APK, как только робот закончит сборку!\n")

while True:
    try:
        # Получаем статус последней сборки
        run_info = subprocess.check_output(["gh", "run", "list", "-L", "1", "--json", "databaseId,status,conclusion"], text=True)
        data = json.loads(run_info)[0]
        status = data["status"]
        
        if status == "completed":
            if data["conclusion"] == "success":
                print("\n\n✅ Сборка успешно завершена! Начинаем скачивание...")
                run_id = str(data["databaseId"])
                subprocess.run(["gh", "run", "download", run_id], check=True)
                print("\n🎉 ГОТОВО! APK скачан.")
                print("Ищи папку с названием сборки (например, app-release) прямо в Termux.")
                break
            else:
                print(f"\n\n❌ Сборка упала с ошибкой: {data['conclusion']}")
                break
        else:
            # Печатаем точки, пока ждем
            print(".", end="", flush=True)
            time.sleep(15)
    except KeyboardInterrupt:
        print("\n\nОстановлено пользователем.")
        break
    except Exception as e:
        print(f"\nОшибка при проверке: {e}")
        break
