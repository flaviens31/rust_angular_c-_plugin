import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable } from 'rxjs';
import { DataModel } from './data-model';

@Injectable({
  providedIn: 'root',
})
export class ApiService {
  private baseUrl = 'http://127.0.0.1:8081/api';

  constructor(private http: HttpClient) {}

  add(a: number, b: number): Observable<number> {
    return this.http.get<number>(`${this.baseUrl}/add/${a}/${b}`);
  }

  printMessage(): Observable<string> {
    return this.http.get<string>(`${this.baseUrl}/message`);
  }

  getDataModel(id: number): Observable<DataModel> {
    return this.http.get<DataModel>(`${this.baseUrl}/data-model/${id}`);
  }
}
