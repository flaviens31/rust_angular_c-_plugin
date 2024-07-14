import { Component } from '@angular/core';
import { RouterOutlet } from '@angular/router';
import { ApiService } from './api.service';
import { NgIf } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { BrowserModule } from '@angular/platform-browser';
import { HttpClientModule } from '@angular/common/http';
import { BrowserAnimationsModule } from '@angular/platform-browser/animations';
import { MatInputModule } from '@angular/material/input';
import { MatButtonModule } from '@angular/material/button';
import { MatToolbarModule } from '@angular/material/toolbar';
import { MatCardModule } from '@angular/material/card';
import { MatIconModule } from '@angular/material/icon';
import { DataModel } from './data-model';
import { catchError, delay, of, retry, tap } from 'rxjs';
@Component({
  selector: 'app-root',
  standalone: true,
  imports: [
    NgIf,
    RouterOutlet,
    FormsModule,
    HttpClientModule,
    MatInputModule,
    MatButtonModule,
    MatCardModule,
    MatToolbarModule,
    MatIconModule,
  ],

  templateUrl: './app.component.html',
  styleUrl: './app.component.css',
})
export class AppComponent {
  title = 'angular-frontend';
  result: number | undefined;
  message: string | undefined;
  modelId: number = 1;
  retryDelay: number = 5000;
  dataModel: DataModel | undefined;
  a: number = 0;
  b: number = 0;
  constructor(private apiService: ApiService) {}
  ngOnInit(): void {
    this.loadDataModel();
  }
  loadDataModel() {
    this.apiService
      .getDataModel(this.modelId)
      .pipe(retry({ delay: this.retryDelay }))
      .subscribe({
        next: (response) => {
          if (response) {
            this.dataModel = response;
          } else {
            // Handle the case where response is null (after retries)
            console.error('Failed to fetch data model after retries');
          }
        },
      });
  }
  addNumbers() {
    this.apiService.add(this.a, this.b).subscribe((result) => {
      this.result = result;
    });
  }

  showMessage() {
    this.apiService.printMessage().subscribe((message) => {
      this.message = message;
    });
  }
}
