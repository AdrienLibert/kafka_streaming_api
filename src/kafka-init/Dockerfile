FROM python:3.12.1-bullseye

RUN mkdir /app
COPY . /app/

WORKDIR /app

RUN pip install -r requirements.txt

CMD ["python3", "main.py"]