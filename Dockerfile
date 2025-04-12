FROM public.ecr.aws/lambda/provided:al2

COPY ./bootstrap /var/task/bootstrap
COPY ./aws-lambda-rie /usr/local/bin/aws-lambda-rie

RUN chmod +x /var/task/bootstrap
