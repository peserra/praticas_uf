#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/time.h>

int vetor_global[] = {1,  2,  3,  4,  5,  6,  7,  8,  9,  10,
                      10, 11, 12, 13, 14, 15, 16, 17, 18, 19};

void *media(void *arg) {
  float soma = 0;
  for (int i = 0; i < 20; i++) {
    soma += vetor_global[i];
  }

  float media = soma / 20; // valor que será retornado
  float *returnMedia = malloc(sizeof(
      returnMedia)); // cria uma alocação de memoria  apontada por returnMedia
  *returnMedia = media; // valor de media recebe valor retornado
  return returnMedia;
}

void *max() {
  int max = 0;
  for (int i = 0; i < 20; i++) {
    if (vetor_global[i] > max)
      max = vetor_global[i];
  }
  int *returnMax = malloc(sizeof(returnMax));
  *returnMax = max;
  return returnMax;
}

void *min() {
  int min = 100;
  for (int i = 0; i < 20; i++) {
    if (vetor_global[i] < min)
      min = vetor_global[i];
  }
  int *returnMin = malloc(sizeof(returnMin));
  *returnMin = min;
  pthread_exit(returnMin);
}

void *moda() {
  int val = 0, count = 0, maxCount = 0;
  for (int i = 0; i < 20; i++) {
    count = 0;
    for (int j = 0; j < 20; j++) {
      if (vetor_global[i] == vetor_global[j]) {
        count++;
      }
    }
    if (count > maxCount) {
      maxCount = count;
      val = vetor_global[i];
    }
  }
  int *returnModa = malloc(sizeof(returnModa));
  *returnModa = val;
  pthread_exit(returnModa);
}

int main() {
  pthread_t threadMedia;
  pthread_t threadMax;
  pthread_t threadMin;
  pthread_t threadModa;

  int return_max;
  int return_min;
  int return_moda;
  float return_media;

  int *return_pointer_max = NULL;
  int *return_pointer_min = NULL;
  int *return_pointer_moda = NULL;
  float *return_pointer_media = NULL;

  struct timeval current_time;
  gettimeofday(&current_time, NULL);
  int inicio = current_time.tv_usec;

  pthread_create(&threadMax, NULL, max, NULL);
  pthread_create(&threadMin, NULL, min, NULL);
  pthread_create(&threadModa, NULL, moda, NULL);
  pthread_create(&threadMedia, NULL, media, NULL);

  pthread_join(threadMax, (void **)&return_pointer_max);
  pthread_join(threadMin, (void **)&return_pointer_min);
  pthread_join(threadModa, (void **)&return_pointer_moda);
  pthread_join(threadMedia, (void **)&return_pointer_media);

  return_max = *return_pointer_max;
  return_min = *return_pointer_min;
  return_moda = *return_pointer_moda;
  return_media = *return_pointer_media;
  free(return_pointer_max);
  free(return_pointer_min);
  free(return_pointer_moda);
  free(return_pointer_media);

  printf("Max do vetor: %d\n", return_max);
  printf("Moda do vetor: %d\n", return_moda);
  printf("Min do vetor: %d\n", return_min);
  printf("Média do vetor: %f\n", return_media);

  gettimeofday(&current_time, NULL);
  int fim = current_time.tv_usec;
  printf("Tempo de execucao com threads: %d Microssegundos", fim - inicio);
}