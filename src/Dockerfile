FROM python:3.11-slim
WORKDIR /app

# install deps
COPY requirements.txt /app/requirements.txt
RUN python -m pip install --no-cache-dir -r requirements.txt

# copy app
COPY src/app.py /app/app.py
COPY src/static /app/static
COPY src/solver.py /app/solver.py

ENV FLASK_APP=app.py
EXPOSE 5000

CMD ["python", "app.py"]
