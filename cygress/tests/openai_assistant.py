import os
from openai import OpenAI

DASHSCOPE_API_KEY = "sk-3697cc7c950143dbb84856531946dc12"
os.environ["DASHSCOPE_API_KEY"] = DASHSCOPE_API_KEY
DASHSCOPE_API_KEY = os.getenv("DASHSCOPE_API_KEY")
if not DASHSCOPE_API_KEY:
    raise ValueError("DASHSCOPE_API_KEY 环境变量设置失败")
# https://dashscope.aliyuncs.com/compatible-mode/v1
def get_response(messages):
    client = OpenAI(
        # 若没有配置环境变量，请用百炼API Key将下行替换为：api_key="sk-xxx",
        api_key=os.getenv("DASHSCOPE_API_KEY"),
        base_url="http://127.0.0.1:6190/compatible-mode/v1",
    )
    try:
        completion = client.chat.completions.create(model="qwen-plus", messages=messages)
    except Exception as e:
        print(e)
        return None
    return completion

messages = [
    {
        "role": "system",
        "content": """你是一名百炼手机商店的店员，你负责给用户推荐手机。手机有两个参数：屏幕尺寸（包括6.1英寸、6.5英寸、6.7英寸）、分辨率（包括2K、4K）。
        你一次只能向用户提问一个参数。如果用户提供的信息不全，你需要反问他，让他提供没有提供的参数。如果参数收集完成，你要说：我已了解您的购买意向，请稍等。""",
    }
]

usage = {
    "input_tokens": 0,
    "output_tokens": 0,
    "total_tokens": 0,
}


assistant_output = "欢迎光临百炼手机商店，您需要购买什么尺寸的手机呢？"
print(f"模型输出：{assistant_output}\n")
while "我已了解您的购买意向" not in assistant_output:
    user_input = input("请输入：")
    messages.append({"role": "user", "content": user_input})
    assistant_output = get_response(messages).choices[0].message.content
    messages.append({"role": "assistant", "content": assistant_output})
    usage["input_tokens"] += get_response(messages).usage.prompt_tokens
    usage["output_tokens"] += get_response(messages).usage.completion_tokens
    usage["total_tokens"] += get_response(messages).usage.total_tokens
    print(f"模型输出：{assistant_output}")
    print(f"输入token：{usage['input_tokens']}")
    print(f"输出token：{usage['output_tokens']}")
    print(f"总token：{usage['total_tokens']}")
    print("\n")